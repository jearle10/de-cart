import * as React from "react";
import { createRoot } from "react-dom/client";

import { Header } from "./components/Header";
import { ProductGrid } from "./components/ProductGrid";
import { Routes, Route, BrowserRouter, Link } from "react-router-dom";
import { _SERVICE } from "../../declarations/de_cart_contract/de_cart_contract.did";
import { useContract, Service } from "./service/service";
import { Home } from "./pages/Home";

import "../assets/main.css";
import { Products } from "./pages/Products";
import { Basket } from "./pages/Basket";
import { Profile } from "./pages/Profile";

const App = () => {
  const [auth, setAuthClient] = React.useState<any>();
  const [id, setId] = React.useState<any>();

  // React.useEffect(() => {
  //   authInternetId(setAuthClient);
  // }, auth);

  return (
    <>
      <BrowserRouter>
        <Header></Header>
        <Routes>
          <Route path="/" element={<Home />}></Route>
          <Route path="/products" element={<Products />}></Route>
          <Route path="/basket" element={<Basket />}></Route>
          <Route path="/profile" element={<Profile />}></Route>
          <Route path="*" element={<Home />}></Route>
        </Routes>
      </BrowserRouter>
    </>
  );
};

const container = document.getElementById("app");
const root = createRoot(container!);
root.render(<App />);
