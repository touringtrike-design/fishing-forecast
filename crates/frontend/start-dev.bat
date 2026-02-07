@echo off
cd /d %~dp0
trunk serve --address 127.0.0.1 --port 3001
pause
