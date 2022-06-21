let programInput = "";
document.getElementById("program-input").onchange = () => {
  programInput = document.getElementById("program-input").value;
  console.log(programInput);
}

document.getElementById("run-program").onclick = () => {
  import("./node_modules/nakala_wasm/nakala_wasm.js").then((js) => {
    let result = js.wasm_interpret(programInput);
    document.getElementById("program-output").value = result;
  });
}
