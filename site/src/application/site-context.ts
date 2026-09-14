import { createContext } from 'react';
import type { SiteRuntime } from './site-contract';

export const SiteContext = createContext<SiteRuntime | null>(null);
