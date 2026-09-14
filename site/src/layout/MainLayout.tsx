import type { ReactNode } from "react";
import { Header } from "./Header";
import { Footer } from "./Footer";

type Props = {
  children: ReactNode;
};

export function MainLayout({ children }: Props) {
  return (
    <div className="app-shell">
      <Header />
      {children}
      <Footer />
    </div>
  );
}