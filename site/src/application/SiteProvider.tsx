import type { ReactNode } from 'react';
import type { SiteRuntime } from './site-contract';
import { SiteContext } from './site-context';

type Props = {
  runtime: SiteRuntime;
  children: ReactNode;
};

export function SiteProvider({ runtime, children }: Props) {
  return <SiteContext.Provider value={runtime}>{children}</SiteContext.Provider>;
}
