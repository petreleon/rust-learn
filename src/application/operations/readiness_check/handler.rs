use std::time::Duration;

use futures::future::join_all;
use futures::FutureExt;
use tokio::time::timeout;

use crate::application::operations::ports::ReadinessDependency;
use crate::application::operations::readiness_check::{
    DependencyCheckOutput, DependencyStatus, ReadinessOutput, ReadinessStatus,
};

const CHECK_TIMEOUT: Duration = Duration::from_secs(3);

pub struct MissingReadinessDependency {
    name: &'static str,
    message: String,
}

impl MissingReadinessDependency {
    pub fn new(name: &'static str, message: impl Into<String>) -> Self {
        Self {
            name,
            message: message.into(),
        }
    }
}

impl ReadinessDependency for MissingReadinessDependency {
    fn name(&self) -> &'static str {
        self.name
    }

    fn check(&mut self) -> futures::future::BoxFuture<'_, Result<(), String>> {
        let message = self.message.clone();
        async move { Err(message) }.boxed()
    }
}

pub async fn check_readiness(dependencies: &mut [Box<dyn ReadinessDependency>]) -> ReadinessOutput {
    let checks = join_all(
        dependencies
            .iter_mut()
            .map(|dependency| check_dependency(dependency.as_mut())),
    )
    .await;

    let ready = checks
        .iter()
        .all(|check| matches!(check.status, DependencyStatus::Ok));

    ReadinessOutput {
        status: if ready {
            ReadinessStatus::Ready
        } else {
            ReadinessStatus::NotReady
        },
        checks,
    }
}

async fn check_dependency(dependency: &mut dyn ReadinessDependency) -> DependencyCheckOutput {
    let name = dependency.name();

    match timeout(CHECK_TIMEOUT, dependency.check()).await {
        Ok(Ok(())) => DependencyCheckOutput {
            name,
            status: DependencyStatus::Ok,
            message: None,
        },
        Ok(Err(message)) => failed(name, message),
        Err(_) => failed(name, "timed out"),
    }
}

fn failed(name: &'static str, message: impl Into<String>) -> DependencyCheckOutput {
    DependencyCheckOutput {
        name,
        status: DependencyStatus::Failed,
        message: Some(message.into()),
    }
}

#[cfg(test)]
mod tests {
    use futures::future::{BoxFuture, FutureExt};

    use super::*;

    struct FakeDependency {
        name: &'static str,
        result: Result<(), String>,
    }

    impl ReadinessDependency for FakeDependency {
        fn name(&self) -> &'static str {
            self.name
        }

        fn check(&mut self) -> BoxFuture<'_, Result<(), String>> {
            let result = self.result.clone();
            async move { result }.boxed()
        }
    }

    #[tokio::test]
    async fn check_readiness_is_ready_when_all_dependencies_pass() {
        let postgres = FakeDependency {
            name: "postgres",
            result: Ok(()),
        };
        let s3 = FakeDependency {
            name: "s3",
            result: Ok(()),
        };

        let output = check_readiness(&mut [Box::new(postgres), Box::new(s3)]).await;

        assert_eq!(output.status, ReadinessStatus::Ready);
        assert_eq!(output.checks.len(), 2);
        assert!(output
            .checks
            .iter()
            .all(|check| matches!(check.status, DependencyStatus::Ok)));
    }

    #[tokio::test]
    async fn check_readiness_reports_not_ready_when_dependency_fails() {
        let postgres = FakeDependency {
            name: "postgres",
            result: Ok(()),
        };
        let s3 = FakeDependency {
            name: "s3",
            result: Err("missing bucket".to_string()),
        };

        let output = check_readiness(&mut [Box::new(postgres), Box::new(s3)]).await;

        assert_eq!(output.status, ReadinessStatus::NotReady);
        assert!(output.checks.iter().any(|check| {
            check.name == "s3"
                && matches!(check.status, DependencyStatus::Failed)
                && check.message.as_deref() == Some("missing bucket")
        }));
    }
}
