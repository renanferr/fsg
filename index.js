// For more comments about what's going on here, check out the `hello_world`
// example.
// import { start } from './pkg'

console.log("Vertex Shader", document.getElementById('vertex-shader').text, "\n")
console.log("Fragment Shader", document.getElementById('fragment-shader').text, "\n")

import('./pkg')
  .catch(console.error);
// start()