import type { NextConfig } from "next";

const apiUrl = process.env.API_URL || "http://127.0.0.1:8080";

const nextConfig: NextConfig = {
  allowedDevOrigins: ["127.0.0.1", "::1"],
  async rewrites() {
    return [
      {
        source: "/health",
        destination: `${apiUrl}/health`,
      },
      {
        source: "/api/:path*",
        destination: `${apiUrl}/api/:path*`,
      },
    ];
  },
};

export default nextConfig;
