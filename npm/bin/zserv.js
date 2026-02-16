#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const https = require('https');
const { execFileSync } = require('child_process');
const AdmZip = require('adm-zip'); // For Windows zip
const tar = require('tar');        // For Unix tar.gz

// Get version from package.json
const pkg = require('../package.json');
const VERSION = pkg.version;
const REPO = 'rinx-dev/zserve'; // Must match GitHub repo
const BIN_NAME = process.platform === 'win32' ? 'zserv.exe' : 'zserv';

// Determine platform and arch
const platform = process.platform;
const arch = process.arch;

let assetName = '';

if (platform === 'win32') {
  if (arch === 'x64') {
    assetName = 'zserv-windows-amd64.zip';
  } else {
    console.error(`Unsupported architecture: ${arch} on Windows`);
    process.exit(1);
  }
} else if (platform === 'darwin') {
  if (arch === 'x64') {
    assetName = 'zserv-macos-amd64.tar.gz';
  } else if (arch === 'arm64') {
    assetName = 'zserv-macos-arm64.tar.gz';
  } else {
    console.error(`Unsupported architecture: ${arch} on macOS`);
    process.exit(1);
  }
} else if (platform === 'linux') {
  if (arch === 'x64') {
    assetName = 'zserv-linux-amd64.tar.gz';
  } else {
    console.error(`Unsupported architecture: ${arch} on Linux`);
    process.exit(1);
  }
} else {
  console.error(`Unsupported platform: ${platform}`);
  process.exit(1);
}

const downloadUrl = `https://github.com/${REPO}/releases/download/v${VERSION}/${assetName}`;
const binPath = path.join(__dirname, BIN_NAME);

// Check if binary already exists
if (fs.existsSync(binPath)) {
  runBinary();
} else {
  console.log(`Downloading zserv ${VERSION} for ${platform}-${arch}...`);
  downloadAndExtract();
}

function downloadAndExtract() {
  const tempFile = path.join(__dirname, assetName);
  const file = fs.createWriteStream(tempFile);

  https.get(downloadUrl, (response) => {
    if (response.statusCode === 302 || response.statusCode === 301) {
      // Follow redirect
      https.get(response.headers.location, (redirectResponse) => {
        handleDownload(redirectResponse, file, tempFile);
      });
    } else {
      handleDownload(response, file, tempFile);
    }
  }).on('error', (err) => {
    console.error(`Error downloading binary: ${err.message}`);
    process.exit(1);
  });
}

function handleDownload(response, file, tempFile) {
  if (response.statusCode !== 200) {
    console.error(`Failed to download binary: HTTP ${response.statusCode}`);
    console.error(`URL: ${downloadUrl}`);
    process.exit(1);
  }

  response.pipe(file);

  file.on('finish', () => {
    file.close(() => {
      extract(tempFile);
    });
  });
}

function extract(tempFile) {
  console.log('Extracting...');
  
  if (assetName.endsWith('.zip')) {
    const zip = new AdmZip(tempFile);
    zip.extractAllTo(__dirname, true);
    fs.unlinkSync(tempFile);
    runBinary();
  } else {
    tar.x({
      file: tempFile,
      cwd: __dirname
    }).then(() => {
      fs.unlinkSync(tempFile);
      runBinary();
    }).catch(err => {
      console.error('Error extracting tarball:', err);
      process.exit(1);
    });
  }
}

function runBinary() {
  // Ensure executable permissions on Unix
  if (platform !== 'win32') {
    try {
      fs.chmodSync(binPath, '755');
    } catch (e) {
      // Ignore if fail
    }
  }

  try {
    // Pass all arguments to the binary
    const args = process.argv.slice(2);
    execFileSync(binPath, args, { stdio: 'inherit' });
  } catch (e) {
    // execFileSync throws if command fails, but stdio: inherit handles output.
    // We just exit with the same code if available.
    if (e.status) {
        process.exit(e.status);
    }
    process.exit(1);
  }
}
