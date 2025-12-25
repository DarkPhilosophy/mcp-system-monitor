#!/bin/bash
set -e

# MCP System Monitor Installation Script
# Supports both homebrew and system rust installations

REPO_URL="https://github.com/DarkPhilosophy/mcp-system-monitor.git"
INSTALL_DIR="/opt/mcp-system-monitor"
SERVICE_NAME="mcp-system-monitor"
BINARY_NAME="mcp-system-monitor"
PORT=57996

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Detect OS
OS="$(uname -s)"
case "$OS" in
    Linux*)     OS="Linux";;
    Darwin*)    OS="macOS";;
    *)          echo "Unsupported OS: $OS"; exit 1;;
esac

echo -e "${GREEN}=== MCP System Monitor Installer ===${NC}"
echo "OS: $OS"

# Check if running as root (for Linux)
if [[ "$OS" == "Linux" ]] && [[ $EUID -ne 0 ]]; then
   echo -e "${RED}This script must be run as root on Linux${NC}"
   exit 1
fi

# Check Rust installation
check_rust() {
    if command -v cargo &> /dev/null; then
        RUST_VERSION=$(cargo --version)
        echo -e "${GREEN}✓ Rust found: $RUST_VERSION${NC}"
        return 0
    fi
    return 1
}

# Install Rust via homebrew (macOS)
install_rust_homebrew() {
    echo -e "${YELLOW}Installing Rust via homebrew...${NC}"
    
    if ! command -v brew &> /dev/null; then
        echo -e "${RED}❌ Homebrew not found. Please install homebrew first.${NC}"
        echo "Visit: https://brew.sh"
        exit 1
    fi
    
    brew install rust
    source $HOME/.cargo/env 2>/dev/null || true
}

# Install Rust via rustup (Linux/manual)
install_rust_rustup() {
    echo -e "${YELLOW}Installing Rust via rustup...${NC}"
    
    if command -v rustup &> /dev/null; then
        echo -e "${GREEN}✓ rustup already installed${NC}"
    else
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source $HOME/.cargo/env
    fi
}

# Check and install Rust
if ! check_rust; then
    echo -e "${YELLOW}Rust not found. Installing...${NC}"
    
    if [[ "$OS" == "macOS" ]]; then
        install_rust_homebrew
    else
        install_rust_rustup
    fi
    
    if ! check_rust; then
        echo -e "${RED}❌ Failed to install Rust${NC}"
        exit 1
    fi
fi

# Clone or update repository
if [ -d "$INSTALL_DIR/.git" ]; then
    echo -e "${YELLOW}Updating repository...${NC}"
    cd "$INSTALL_DIR"
    git pull origin master
else
    echo -e "${YELLOW}Cloning repository...${NC}"
    mkdir -p "$(dirname $INSTALL_DIR)"
    git clone "$REPO_URL" "$INSTALL_DIR"
    cd "$INSTALL_DIR"
fi

# Build the project
echo -e "${YELLOW}Building project...${NC}"
cd "$INSTALL_DIR"
cargo build --release

BINARY_PATH="$INSTALL_DIR/target/release/$BINARY_NAME"

if [ ! -f "$BINARY_PATH" ]; then
    echo -e "${RED}❌ Build failed. Binary not found at $BINARY_PATH${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Build successful${NC}"

# Create systemd service (Linux only)
if [[ "$OS" == "Linux" ]]; then
    echo -e "${YELLOW}Setting up systemd service...${NC}"
    
    SERVICE_FILE="/etc/systemd/system/${SERVICE_NAME}.service"
    
    cat > "$SERVICE_FILE" << EOF
[Unit]
Description=MCP System Monitor Server
Documentation=https://github.com/DarkPhilosophy/mcp-system-monitor
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=root
Group=root
WorkingDirectory=$INSTALL_DIR
ExecStart=$BINARY_PATH
Restart=on-failure
RestartSec=5
StandardOutput=journal
StandardError=journal
Environment="RUST_LOG=info"
Environment="RUST_BACKTRACE=1"

[Install]
WantedBy=multi-user.target
EOF
    
    # Reload systemd and enable service
    systemctl daemon-reload
    systemctl enable "$SERVICE_NAME.service"
    
    echo -e "${GREEN}✓ Systemd service created${NC}"
    echo -e "${GREEN}✓ Service enabled${NC}"
    
    # Start service
    echo -e "${YELLOW}Starting service...${NC}"
    systemctl start "$SERVICE_NAME.service"
    
    # Check status
    if systemctl is-active --quiet "$SERVICE_NAME.service"; then
        echo -e "${GREEN}✓ Service is running${NC}"
        systemctl status "$SERVICE_NAME.service" --no-pager | head -10
    else
        echo -e "${RED}❌ Service failed to start${NC}"
        systemctl status "$SERVICE_NAME.service"
        exit 1
    fi
fi

# Create launchd service (macOS)
if [[ "$OS" == "macOS" ]]; then
    echo -e "${YELLOW}Setting up launchd service...${NC}"
    
    LAUNCHD_DIR="$HOME/Library/LaunchAgents"
    mkdir -p "$LAUNCHD_DIR"
    
    PLIST_FILE="$LAUNCHD_DIR/com.mcp.system-monitor.plist"
    
    cat > "$PLIST_FILE" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.mcp.system-monitor</string>
    <key>ProgramArguments</key>
    <array>
        <string>BINARY_PATH_PLACEHOLDER</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/var/log/mcp-system-monitor.log</string>
    <key>StandardErrorPath</key>
    <string>/var/log/mcp-system-monitor.log</string>
    <key>WorkingDirectory</key>
    <string>INSTALL_DIR_PLACEHOLDER</string>
    <key>EnvironmentVariables</key>
    <dict>
        <key>RUST_LOG</key>
        <string>info</string>
    </dict>
</dict>
</plist>
EOF
    
    # Replace placeholders
    sed -i '' "s|BINARY_PATH_PLACEHOLDER|$BINARY_PATH|g" "$PLIST_FILE"
    sed -i '' "s|INSTALL_DIR_PLACEHOLDER|$INSTALL_DIR|g" "$PLIST_FILE"
    
    # Load service
    launchctl load "$PLIST_FILE" 2>/dev/null || launchctl unload "$PLIST_FILE" 2>/dev/null; launchctl load "$PLIST_FILE"
    
    echo -e "${GREEN}✓ Launchd service created at $PLIST_FILE${NC}"
    
    # Wait and check
    sleep 2
    if launchctl list | grep -q "com.mcp.system-monitor"; then
        echo -e "${GREEN}✓ Service is running${NC}"
    else
        echo -e "${YELLOW}⚠ Service may not be running. Check logs at /var/log/mcp-system-monitor.log${NC}"
    fi
fi

# Test API
echo -e "${YELLOW}Testing API health...${NC}"
sleep 2

for i in {1..5}; do
    if curl -s http://localhost:$PORT/health > /dev/null 2>&1; then
        HEALTH=$(curl -s http://localhost:$PORT/health)
        echo -e "${GREEN}✓ API is responding${NC}"
        echo "Health: $HEALTH"
        break
    fi
    
    if [ $i -lt 5 ]; then
        echo -e "${YELLOW}Waiting for service to start... ($i/5)${NC}"
        sleep 1
    else
        echo -e "${YELLOW}⚠ API not responding yet (this is normal on first start)${NC}"
    fi
done

# Summary
echo ""
echo -e "${GREEN}=== Installation Complete ===${NC}"
echo ""
echo "📍 Installation Directory: $INSTALL_DIR"
echo "🔧 Binary: $BINARY_PATH"
echo "🌐 API Port: $PORT"
echo ""

if [[ "$OS" == "Linux" ]]; then
    echo "📋 Service Management (Linux):"
    echo "   View status:   sudo systemctl status $SERVICE_NAME"
    echo "   View logs:     sudo journalctl -u $SERVICE_NAME -f"
    echo "   Stop service:  sudo systemctl stop $SERVICE_NAME"
    echo "   Start service: sudo systemctl start $SERVICE_NAME"
    echo "   Restart:       sudo systemctl restart $SERVICE_NAME"
else
    echo "📋 Service Management (macOS):"
    echo "   View logs:     tail -f /var/log/mcp-system-monitor.log"
    echo "   Unload:        launchctl unload ~/Library/LaunchAgents/com.mcp.system-monitor.plist"
    echo "   Load:          launchctl load ~/Library/LaunchAgents/com.mcp.system-monitor.plist"
fi

echo ""
echo "🧪 Test API:"
echo "   curl http://localhost:$PORT/health"
echo "   curl http://localhost:$PORT/api/system/info"
echo ""
echo -e "${GREEN}✓ Setup complete! MCP System Monitor is ready to use.${NC}"
