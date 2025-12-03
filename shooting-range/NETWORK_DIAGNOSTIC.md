# WSL Network Configuration Diagnostic Guide

## Quick Diagnostic

Run the diagnostic script from **within WSL** (not PowerShell):

```bash
cd /home/rana/active/macky-shooting-range/shooting-range
chmod +x check-network.sh
./check-network.sh
```

## Common WSL Network Issues

### 1. DNS Resolution Problems

**Symptoms:**
- `npm install` fails with network errors
- Cannot resolve registry.npmjs.org
- Timeout errors

**Check DNS:**
```bash
cat /etc/resolv.conf
```

**Common Issues:**
- `/etc/resolv.conf` is auto-generated and may point to wrong DNS
- DNS servers may be unreachable

**Fix:**
```bash
# Backup current config
sudo cp /etc/resolv.conf /etc/resolv.conf.backup

# Create new resolv.conf with Google DNS
sudo bash -c 'cat > /etc/resolv.conf << EOF
nameserver 8.8.8.8
nameserver 8.8.4.4
EOF'

# Make it persistent (prevent WSL from overwriting)
sudo chattr +i /etc/resolv.conf
```

**Alternative - Use WSL config:**
Create or edit `/etc/wsl.conf`:
```bash
sudo nano /etc/wsl.conf
```

Add:
```ini
[network]
generateResolvConf = false
```

Then create `/etc/resolv.conf` manually as above.

### 2. Proxy Configuration Issues

**Check if you're behind a proxy:**
```bash
echo $HTTP_PROXY
echo $HTTPS_PROXY
```

**If proxy is needed, configure npm:**
```bash
npm config set proxy http://proxy-server:port
npm config set https-proxy http://proxy-server:port
```

**If proxy is NOT needed but is set:**
```bash
npm config delete proxy
npm config delete https-proxy
unset HTTP_PROXY
unset HTTPS_PROXY
unset http_proxy
unset https_proxy
```

### 3. npm Registry Issues

**Check current registry:**
```bash
npm config get registry
```

**Test connectivity:**
```bash
curl -I https://registry.npmjs.org
```

**If registry is blocked or slow, try alternative:**
```bash
npm config set registry https://registry.npmjs.org/
# Or use a mirror:
npm config set registry https://registry.npmmirror.com
```

### 4. WSL Network Mode Issues

**Check WSL network configuration:**
```bash
cat /etc/wsl.conf
```

**If file doesn't exist, create it:**
```bash
sudo nano /etc/wsl.conf
```

Add:
```ini
[network]
generateResolvConf = false
```

**Restart WSL after changes:**
From Windows PowerShell:
```powershell
wsl --shutdown
# Then restart WSL
```

### 5. Firewall/Windows Defender Issues

**Check if Windows Firewall is blocking:**
- Open Windows Defender Firewall
- Check if WSL network access is allowed
- Temporarily disable to test (re-enable after)

### 6. IPv6 Issues

**If IPv6 is causing problems:**
```bash
# Disable IPv6 in /etc/wsl.conf
[network]
generateResolvConf = false

# Then in /etc/resolv.conf, use IPv4 only:
nameserver 8.8.8.8
nameserver 8.8.4.4
options single-request-reopen
```

## Step-by-Step Diagnostic

1. **Test basic connectivity:**
   ```bash
   ping -c 3 8.8.8.8
   ping -c 3 google.com
   ```

2. **Check DNS:**
   ```bash
   nslookup registry.npmjs.org
   ```

3. **Test npm registry:**
   ```bash
   curl -v https://registry.npmjs.org
   ```

4. **Check npm config:**
   ```bash
   npm config list
   ```

5. **Try npm install with verbose output:**
   ```bash
   npm install --verbose
   ```

## Quick Fixes

### Fix 1: Reset DNS
```bash
sudo rm /etc/resolv.conf
sudo bash -c 'echo "nameserver 8.8.8.8" > /etc/resolv.conf'
sudo bash -c 'echo "nameserver 8.8.4.4" >> /etc/resolv.conf'
```

### Fix 2: Clear npm cache
```bash
npm cache clean --force
```

### Fix 3: Reset npm config
```bash
npm config delete proxy
npm config delete https-proxy
npm config set registry https://registry.npmjs.org/
```

### Fix 4: Use different network mode
From Windows PowerShell (as Administrator):
```powershell
# Check current WSL version
wsl --list --verbose

# If using WSL2, try switching to WSL1 temporarily to test
wsl --set-version Ubuntu 1
```

## Important: Run npm from WSL

**CRITICAL:** Always run `npm install` from **within WSL**, not from Windows PowerShell. The UNC path issue you're seeing happens when npm runs from Windows but tries to access WSL files.

**Correct way:**
1. Open WSL/Ubuntu terminal
2. Navigate to project: `cd /home/rana/active/macky-shooting-range/shooting-range`
3. Run: `npm install`

**Wrong way (causes UNC path errors):**
- Running `npm install` from Windows PowerShell
- Running `npm install` while accessing files via `\\wsl.localhost\...`

