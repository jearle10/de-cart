import * as React from "react";
import { useEffect } from "react";
import { ProductGrid } from "../components/ProductGrid";
import { Service, useContract, useEncryptionKey } from "../service/service";

export function Products() {
  const { loading, products } = useContract("get_all_products" as Service);
  const { key } = useEncryptionKey("ibe_encryption_key" as Service);

  // Transport key
  const tsk_seed = window.crypto.getRandomValues(new Uint8Array(32));

  console.log(tsk_seed);

  return (
    <div>
      {loading ? "loading..." : <ProductGrid data={products}></ProductGrid>}
    </div>
  );
}
