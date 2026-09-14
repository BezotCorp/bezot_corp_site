import { Route, Routes } from 'react-router-dom';
import { SiteProvider } from './application/SiteProvider';
import type { SiteRuntime } from './application/site-contract';
import { PageRenderer } from './pages/PageRenderer';

type Props = {
  runtime: SiteRuntime;
};

export function App({ runtime }: Props) {
  return (
    <SiteProvider runtime={runtime}>
      <Routes>
        <Route path="/" element={<PageRenderer />} />
        <Route path="/*" element={<PageRenderer />} />
      </Routes>
    </SiteProvider>
  );
}
