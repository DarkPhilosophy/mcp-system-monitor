#!/bin/bash

# MCP System Monitor Deployment Script
# This script deploys the MCP System Monitor on a Linux server

set -e

echo "🚀 MCP System Monitor Deployment Script"
echo "========================================"

# Check if running as root
if [[ $EUID -eq 0 ]]; then
   echo "❌ This script should not be run as root"
   exit 1
fi

# Check Linux distribution
if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS=$NAME
    VER=$VERSION_ID
else
    echo "❌ Cannot determine Linux distribution"
    exit 1
fi

echo "📋 Detected OS: $OS $VER"

# Install dependencies based on distribution
echo "📦 Installing dependencies..."

if [[ "$OS" == *"Ubuntu"* ]] || [[ "$OS" == *"Debian"* ]]; then
    sudo apt-get update
    sudo apt-get install -y curl docker.io docker-compose build-essential
    sudo systemctl enable docker
    sudo systemctl start docker
    sudo usermod -aG docker $USER
elif [[ "$OS" == *"CentOS"* ]] || [[ "$OS" == *"Red Hat"* ]]; then
    sudo yum update -y
    sudo yum install -y curl docker docker-compose gcc
    sudo systemctl enable docker
    sudo systemctl start docker
    sudo usermod -aG docker $USER
else
    echo "⚠️  Unsupported distribution. Please install Docker manually."
fi

# Create application directory
echo "📁 Creating application directory..."
mkdir -p ~/mcp-system-monitor
cd ~/mcp-system-monitor

# Download application files
echo "⬇️  Downloading application files..."
curl -L -o docker-compose.yml https://raw.githubusercontent.com/your-repo/mcp-system-monitor/main/docker-compose.yml
curl -L -o Dockerfile https://raw.githubusercontent.com/your-repo/mcp-system-monitor/main/Dockerfile

# Create systemd service file
echo "🔧 Creating systemd service..."
sudo tee /etc/systemd/system/mcp-system-monitor.service > /dev/null <<EOF
[Unit]
Description=MCP System Monitor
After=docker.service
Requires=docker.service

[Service]
Type=oneshot
RemainAfterExit=yes
WorkingDirectory=/home/$USER/mcp-system-monitor
ExecStart=/usr/bin/docker-compose up -d
ExecStop=/usr/bin/docker-compose down
TimeoutStartSec=0

[Install]
WantedBy=multi-user.target
EOF

# Reload systemd and enable service
sudo systemctl daemon-reload
sudo systemctl enable mcp-system-monitor

echo "✅ Installation completed!"
echo ""
echo "🎯 Next steps:"
echo "1. Logout and login again to apply docker group changes"
echo "2. Start the service: sudo systemctl start mcp-system-monitor"
echo "3. Check status: sudo systemctl status mcp-system-monitor"
echo "4. View logs: sudo journalctl -u mcp-system-monitor -f"
echo "5. Test API: curl http://localhost:8080/health"
echo ""
echo "🌐 The MCP server will be available at:"
echo "   - HTTP API: http://$(hostname -I | awk '{print $1}'):8080"
echo "   - Health check: http://$(hostname -I | awk '{print $1}'):8080/health"
echo ""
echo "📚 For AI agent integration, use the MCP protocol endpoints:"
echo "   - System info: GET /api/system/info"
echo "   - CPU info: GET /api/system/cpu"
echo "   - Memory info: GET /api/system/memory"
echo "   - Process list: GET /api/system/processes"
echo ""
echo "🔒 Security note: This service runs with privileged access to monitor system resources."
echo "   Consider firewall rules and network security for production deployments." 