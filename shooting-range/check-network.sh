#!/bin/bash

echo "=== WSL Network Configuration Diagnostic ==="
echo ""

echo "1. Testing Internet Connectivity:"
echo "   - Testing DNS resolution..."
if ping -c 2 google.com > /dev/null 2>&1; then
    echo "   ✓ DNS resolution works (google.com)"
else
    echo "   ✗ DNS resolution FAILED (google.com)"
fi

echo "   - Testing direct IP connectivity..."
if ping -c 2 8.8.8.8 > /dev/null 2>&1; then
    echo "   ✓ Direct IP connectivity works (8.8.8.8)"
else
    echo "   ✗ Direct IP connectivity FAILED (8.8.8.8)"
fi

echo ""
echo "2. DNS Configuration (/etc/resolv.conf):"
cat /etc/resolv.conf
echo ""

echo "3. Environment Proxy Variables:"
echo "   HTTP_PROXY: ${HTTP_PROXY:-not set}"
echo "   HTTPS_PROXY: ${HTTPS_PROXY:-not set}"
echo "   http_proxy: ${http_proxy:-not set}"
echo "   https_proxy: ${https_proxy:-not set}"
echo "   NO_PROXY: ${NO_PROXY:-not set}"
echo "   no_proxy: ${no_proxy:-not set}"
echo ""

echo "4. npm Configuration:"
if command -v npm &> /dev/null; then
    echo "   npm version: $(npm --version)"
    echo "   npm registry: $(npm config get registry)"
    echo "   npm proxy: $(npm config get proxy || echo 'not set')"
    echo "   npm https-proxy: $(npm config get https-proxy || echo 'not set')"
    echo "   npm strict-ssl: $(npm config get strict-ssl)"
else
    echo "   ✗ npm not found in PATH"
fi
echo ""

echo "5. Network Interfaces:"
ip addr show | grep -E "^[0-9]+:|inet " | head -10
echo ""

echo "6. Default Gateway:"
ip route | grep default || echo "   No default gateway found"
echo ""

echo "7. Testing npm registry connectivity:"
if command -v npm &> /dev/null; then
    REGISTRY=$(npm config get registry)
    echo "   Testing connection to: $REGISTRY"
    if curl -s --connect-timeout 5 "$REGISTRY" > /dev/null 2>&1; then
        echo "   ✓ Can reach npm registry"
    else
        echo "   ✗ Cannot reach npm registry"
    fi
else
    echo "   npm not available for testing"
fi
echo ""

echo "8. WSL Network Mode Check:"
if [ -f /etc/wsl.conf ]; then
    echo "   /etc/wsl.conf exists:"
    cat /etc/wsl.conf
else
    echo "   /etc/wsl.conf does not exist"
fi
echo ""

echo "=== Diagnostic Complete ==="

