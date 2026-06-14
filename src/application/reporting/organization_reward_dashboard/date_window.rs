use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OrganizationRewardDashboardDateWindow {
    starts_at: Option<NaiveDateTime>,
    ends_at: Option<NaiveDateTime>,
}

impl OrganizationRewardDashboardDateWindow {
    pub(crate) fn starts_at(self) -> Option<NaiveDateTime> {
        self.starts_at
    }

    pub(crate) fn ends_at(self) -> Option<NaiveDateTime> {
        self.ends_at
    }
}

pub(crate) fn organization_reward_dashboard_date_window(
    from: Option<NaiveDate>,
    to: Option<NaiveDate>,
) -> OrganizationRewardDashboardDateWindow {
    OrganizationRewardDashboardDateWindow {
        starts_at: from.map(start_of_day),
        ends_at: to.map(end_of_day),
    }
}

fn start_of_day(date: NaiveDate) -> NaiveDateTime {
    NaiveDateTime::new(date, NaiveTime::from_hms_opt(0, 0, 0).unwrap())
}

fn end_of_day(date: NaiveDate) -> NaiveDateTime {
    NaiveDateTime::new(date, NaiveTime::from_hms_opt(23, 59, 59).unwrap())
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::organization_reward_dashboard_date_window;

    #[test]
    fn expands_optional_dates_to_inclusive_day_bounds() {
        let from = NaiveDate::from_ymd_opt(2026, 1, 2).unwrap();
        let to = NaiveDate::from_ymd_opt(2026, 1, 5).unwrap();

        let window = organization_reward_dashboard_date_window(Some(from), Some(to));

        assert_eq!(
            window.starts_at().unwrap(),
            from.and_hms_opt(0, 0, 0).unwrap()
        );
        assert_eq!(
            window.ends_at().unwrap(),
            to.and_hms_opt(23, 59, 59).unwrap()
        );
    }

    #[test]
    fn keeps_open_ended_bounds_absent() {
        let window = organization_reward_dashboard_date_window(None, None);

        assert_eq!(window.starts_at(), None);
        assert_eq!(window.ends_at(), None);
    }
}
