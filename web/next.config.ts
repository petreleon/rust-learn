import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  async rewrites() {
    return [
      {
        source: '/api/:path*',
        destination: `${process.env.API_URL || 'http://rust-app:8080'}/:path*`,
      },
    ];
  },
};

export default nextConfig;
