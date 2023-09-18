import { de_cart_contract } from "../../declarations/de_cart_contract";

document.querySelector("form").addEventListener("submit", async (e) => {
  e.preventDefault();
  const button = e.target.querySelector("button");

  const name = document.getElementById("name").value.toString();

  button.setAttribute("disabled", true);

  // Interact with foo actor, calling the greet method
  // const greeting = await de_cart_contract.

  button.removeAttribute("disabled");

  // document.getElementById("greeting").innerText = greeting;

  return false;
});
