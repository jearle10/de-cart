import { de_cart_contract } from "../../declarations/de_cart_contract";
import * as React from "react";
import { createRoot } from 'react-dom/client'

import '../assets/main.css';

const App = () => {
    
  return (
      <div>
        <div className="bg-slate-200">
            De-cart homepage
        </div>
      </div>
  );
};

const container = document.getElementById('app')
const root = createRoot(container!)
root.render(<App/>);