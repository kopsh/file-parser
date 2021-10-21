const ipc = require('electron').ipcRenderer
const rustLib = require('./file-parser-rust/index.node');


window.addEventListener('DOMContentLoaded', () => {
	const btn = document.getElementById("operate-btn");
	const bankFile = document.getElementById("select-bank-file");
	bankFile.addEventListener('click', function (event) {
		ipc.send('open-file-dialog')
	});
	//Getting back the information after selecting the file
	ipc.on('selected-file', function (event, path) {
		//do what you want with the path/file selected, for example:
		document.getElementById('selected-file').innerHTML = `You selected: ${path}`;
		alert(rustLib.parse_files(path));
	});
	// btn.addEventListener("click", () => {
	// 	if (!bankFile.value) {
	// 		alert("没有选择银行文件");
	// 		return
	// 	}
	// 	alert(bankFile.files[0].name);
	// })
})