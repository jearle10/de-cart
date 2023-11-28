import * as React from "react";

export function ProductGrid({ data }: any) {
  const images = [
    "https://picsum.photos/seed/1/200/300",
    "https://picsum.photos/seed/2/200/300",
    "https://picsum.photos/seed/3/200/300",
    "https://picsum.photos/seed/4/200/300",
    "https://picsum.photos/seed/5/200/300",
    "https://picsum.photos/seed/6/200/300",
  ];

  let products = data.products[0][1].map((entry: any) => entry[1]);
  console.log(products);

  return (
    <main className="p-6 bg-gray-100 dark:bg-zinc-800">
      <div className="grid grid-cols-4 gap-6">
        {products.map((product: any) => (
          <div
            key={product.sku}
            className="flex flex-col items-center bg-white dark:bg-zinc-900 p-4 rounded-md shadow-lg"
          >
            <img
              alt="Product Image"
              className="object-cover rounded-md"
              height="200"
              src={"https://picsum.photos/seed/1/200/300"}
              style={{
                aspectRatio: "200/200",
                objectFit: "cover",
              }}
              width="200"
            />
            <h2 className="mt-4 text-lg font-semibold text-zinc-900 dark:text-zinc-100">
              {product.sku}
            </h2>
            <p className="mt-2 text-zinc-600 dark:text-zinc-300">
              {product.description}
            </p>
            <span className="mt-2 text-lg font-bold text-zinc-900 dark:text-zinc-100">
              $99.99
            </span>
          </div>
        ))}
      </div>
    </main>
  );
}
