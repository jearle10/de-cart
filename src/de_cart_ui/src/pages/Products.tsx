import * as React from "react";
import { ProductGrid } from "../components/ProductGrid";
import { Service, useContract } from "../service/service";

export function Products() {
  const { loading, products } = useContract("get_all_products" as Service);

  return (
    <div>
      {loading ? "loading..." : <ProductGrid data={products}></ProductGrid>}
    </div>
  );
}
