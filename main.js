const {
	app,
	dialog,
	BrowserWindow
} = require("electron");
const path = require('path');


function createWindow() {
	const win = new BrowserWindow({
		width: 800,
		height: 600,
		webPreferences: {
			preload: path.join(__dirname, 'preload.js')
		}
	});

	const ipc = require('electron').ipcMain
	const dialog = require('electron').dialog
	ipc.on('open-file-dialog', function (event) {
		dialog.showOpenDialog(win, {
			properties: ['openFile']
		}).then(result => {
			event.sender.send('selected-file', result.filePaths[0])
		}).catch(err => {
			console.log(err);
		})
	})

	win.loadFile('index.html');
}

// 监听视窗关闭的事件（在Mac OS 系统下是不会触发该事件的）
app.on('window-all-closed', () => {
	if (process.platform !== 'darwin') {
		app.quit();
	}
});

app.whenReady().then(() => {
	createWindow()
})