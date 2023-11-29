import * as React from "react";
import { useEffect } from "react";
import { ProductGrid } from "../components/ProductGrid";
import {
  Service,
  useContract,
  useEncryptionKey,
  useEncryption,
} from "../service/service";

export function Products() {
  const { loading, products } = useContract("get_all_products" as Service);
  // const { key } = useEncryptionKey("ibe_encryption_key" as Service);
  const { cipherText } = useEncryption("hello");

  return (
    <div>
      {loading ? "loading..." : <ProductGrid data={products}></ProductGrid>}
    </div>
  );
}
