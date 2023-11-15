import * as React from "react";
import { Link } from "react-router-dom";

export function Header() {
  return (
    <header className="flex items-center justify-between px-6 py-4 bg-white dark:bg-zinc-900">
      <div className="flex items-center space-x-4">
        <svg
          className=" w-8 h-8 text-zinc-900 dark:text-zinc-100"
          fill="none"
          height="24"
          stroke="currentColor"
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth="2"
          viewBox="0 0 24 24"
          width="24"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path d="m2 7 4.41-4.41A2 2 0 0 1 7.83 2h8.34a2 2 0 0 1 1.42.59L22 7" />
          <path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8" />
          <path d="M15 22v-4a2 2 0 0 0-2-2h-2a2 2 0 0 0-2 2v4" />
          <path d="M2 7h20" />
          <path d="M22 7v3a2 2 0 0 1-2 2v0a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 16 12a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 12 12a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 8 12a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 4 12v0a2 2 0 0 1-2-2V7" />
        </svg>
        <span className="text-2xl font-semibold text-zinc-900 dark:text-zinc-100">
          3cart
        </span>
      </div>

      <nav className="space-x-4 flex">
        <li className="list-none">
          <Link to="/">Home</Link>
        </li>
        <li className="list-none">
          <Link to="/products"> Products </Link>
        </li>
        <li className="list-none">
          <Link to="/basket"> Basket </Link>
        </li>
        <li className="list-none">
          <Link to="/basket"> Profile </Link>
        </li>

        {/* <Link to="/products">Products</Link> */}
      </nav>
      <button className="bg-black hover:bg-gray-600 text-white rounded-md px-4 py-1">
        Login
      </button>
    </header>
  );
}
