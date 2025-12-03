# Installing Dependencies

The npm dependencies need to be installed. Due to Windows/WSL path issues, please run this command **in WSL** (not from Windows PowerShell):

```bash
cd shooting-range
npm install
```

Or if you're already in the shooting-range directory:

```bash
npm install
```

This will install:
- vue-router
- pinia
- And all other dependencies

After installation, you should see a `node_modules` directory created, and the Vite error should be resolved.

## Alternative: Install specific packages

If the full `npm install` fails, you can try installing just the missing packages:

```bash
npm install vue-router@^4.2.5 pinia@^2.1.7
```

## Troubleshooting

If you encounter path issues:
1. Make sure you're running the command in WSL (Ubuntu terminal), not Windows PowerShell
2. Try using the full path: `cd /home/rana/active/macky-shooting-range/shooting-range && npm install`
3. If node_modules already exists but is corrupted, try: `rm -rf node_modules package-lock.json && npm install`

