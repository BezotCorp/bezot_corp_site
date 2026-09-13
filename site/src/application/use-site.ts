import { useContext } from 'react';
import { SiteContext } from './site-context';

export function useSite() {
  const runtime = useContext(SiteContext);

  if (!runtime) {
    throw new Error('SiteProvider is missing from the application tree');
  }

  return runtime;
}
