# WSL Network Configuration Issue - Analysis & Fix

## Problem Identified

From the error logs, there are **two separate issues**:

### Issue 1: UNC Path Problem (Primary)
```
npm error UNC paths are not supported. Defaulting to Windows directory.
```

**Root Cause:** npm is running from **Windows** but trying to access files in **WSL** via UNC paths (`\\wsl.localhost\...`). Windows CMD doesn't support UNC paths as current directories.

**Solution:** You MUST run `npm install` from **within WSL**, not from Windows PowerShell.

### Issue 2: Potential Network Configuration Issue
If npm still fails after running from WSL, it's likely a network/DNS configuration problem in WSL.

## Immediate Fix Steps

### Step 1: Run npm from WSL (Not Windows)

1. **Open WSL/Ubuntu terminal** (not PowerShell)
2. Navigate to project:
   ```bash
   cd /home/rana/active/macky-shooting-range/shooting-range
   ```
3. Run npm install:
   ```bash
   npm install
   ```

### Step 2: If Step 1 Still Fails - Check Network Config

Run the diagnostic script:
```bash
cd /home/rana/active/macky-shooting-range/shooting-range
chmod +x check-network.sh
./check-network.sh
```

### Step 3: Common Network Fixes

#### Fix DNS Resolution:
```bash
# Check current DNS
cat /etc/resolv.conf

# If DNS is wrong or missing, fix it:
sudo bash -c 'cat > /etc/resolv.conf << EOF
nameserver 8.8.8.8
nameserver 8.8.4.4
EOF'

# Prevent WSL from overwriting it
sudo chattr +i /etc/resolv.conf
```

#### Fix WSL Network Configuration:
```bash
# Create/edit WSL config
sudo nano /etc/wsl.conf
```

Add this content:
```ini
[network]
generateResolvConf = false
```

Then restart WSL from Windows PowerShell:
```powershell
wsl --shutdown
```

#### Check Proxy Settings:
```bash
# Check if proxy is set (should be empty if not needed)
echo $HTTP_PROXY
echo $HTTPS_PROXY

# If proxy is incorrectly set, unset it:
unset HTTP_PROXY
unset HTTPS_PROXY
unset http_proxy
unset https_proxy

# Also check npm proxy config:
npm config get proxy
npm config get https-proxy

# If set incorrectly, remove:
npm config delete proxy
npm config delete https-proxy
```

#### Test Network Connectivity:
```bash
# Test basic connectivity
ping -c 3 8.8.8.8

# Test DNS
ping -c 3 google.com

# Test npm registry
curl -I https://registry.npmjs.org
```

## Quick Diagnostic Commands

Run these in WSL to identify the issue:

```bash
# 1. Check DNS
cat /etc/resolv.conf

# 2. Test connectivity
ping -c 2 8.8.8.8
ping -c 2 google.com

# 3. Test npm registry
curl -v https://registry.npmjs.org

# 4. Check npm config
npm config list

# 5. Check proxy env vars
env | grep -i proxy

# 6. Check network interfaces
ip addr show

# 7. Check default gateway
ip route | grep default
```

## Most Likely Solutions

Based on common WSL network issues:

1. **DNS not resolving:** Fix `/etc/resolv.conf` (see Step 3 above)
2. **WSL overwriting DNS:** Configure `/etc/wsl.conf` to prevent auto-generation
3. **Proxy misconfiguration:** Clear proxy settings if not needed
4. **npm registry blocked:** Try alternative registry or check firewall

## After Fixing

Once network is fixed, try npm install again:
```bash
cd /home/rana/active/macky-shooting-range/shooting-range
npm cache clean --force
npm install
```

## Still Having Issues?

If problems persist after trying the above:

1. Check Windows Firewall settings
2. Check if corporate proxy is required
3. Try using a different DNS (1.1.1.1, 8.8.8.8)
4. Check WSL version: `wsl --list --verbose`
5. Consider switching WSL network mode

