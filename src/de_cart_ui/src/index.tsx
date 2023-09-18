import { de_cart_contract } from "../../declarations/de_cart_contract";
import * as React from "react";
import { render } from "react-dom";

const MyHello = () => {
  const [name, setName] = React.useState('');
  const [message, setMessage] = React.useState('');

  async function doGreet() {
    console.log("Submitted a greeting")
  }

  return (
      <div style={{ "fontSize": "30px" }}>
        <div style={{ "backgroundColor": "yellow" }}>
          <p>Greetings, from DFINITY!</p>
          <p>
            {" "}
            Type your message in the Name input field, then click{" "}
            <b> Get Greeting</b> to display the result.
          </p>
        </div>
        <div style={{ margin: "30px" }}>
          <input
              id="name"
              value={name}
              onChange={(e) => console.log(e.target.value)}
          ></input>
          <button onClick={doGreet}>Get Greeting!</button>
        </div>
        <div>
          Greeting is: "
          <span style={{ color: "blue" }}>{message}</span>"
        </div>
      </div>
  );
};

render(<MyHello />, document.getElementById("app"));