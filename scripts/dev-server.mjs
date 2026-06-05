/**
 * Dev Server for Orbit System Manager
 *
 * Simple static file server for Tauri development mode.
 * Tauri v2's `beforeDevCommand` runs this script, waits for the
 * server to be ready, then opens the WebView window.
 * When the app closes, Tauri automatically kills this process.
 */

import http from 'node:http';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, '..');
const port = 3000;

const MIME_TYPES = {
  '.html': 'text/html; charset=utf-8',
  '.js':   'application/javascript; charset=utf-8',
  '.css':  'text/css; charset=utf-8',
  '.json': 'application/json; charset=utf-8',
  '.png':  'image/png',
  '.ico':  'image/x-icon',
  '.svg':  'image/svg+xml',
  '.map':  'application/json',
};

const server = http.createServer((req, res) => {
  // Map URL to file path
  let filePath = req.url === '/' ? '/index.html' : req.url;
  filePath = path.join(root, filePath);

  // Security: ensure we don't serve files outside the project root
  if (!filePath.startsWith(root)) {
    res.writeHead(403);
    res.end('Forbidden');
    return;
  }

  fs.readFile(filePath, (err, data) => {
    if (err) {
      // If file not found, serve index.html (SPA fallback)
      if (err.code === 'ENOENT' && !path.extname(req.url)) {
        const indexPath = path.join(root, 'index.html');
        fs.readFile(indexPath, (err2, data2) => {
          if (err2) {
            res.writeHead(404);
            res.end('Not found');
            return;
          }
          res.writeHead(200, { 'Content-Type': 'text/html; charset=utf-8' });
          res.end(data2);
        });
        return;
      }
      res.writeHead(404);
      res.end('Not found');
      return;
    }

    const ext = path.extname(filePath);
    const contentType = MIME_TYPES[ext] || 'application/octet-stream';
    res.writeHead(200, { 'Content-Type': contentType });
    res.end(data);
  });
});

server.listen(port, () => {
  console.log(`[dev-server] Serving at http://localhost:${port}`);
  console.log(`[dev-server] Root: ${root}`);
});

// Graceful shutdown
process.on('SIGTERM', () => {
  server.close();
  process.exit(0);
});
