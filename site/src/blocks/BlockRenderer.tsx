import type { ContentBlock } from '../application/site-contract';
import { useSite } from '../application/use-site';

type Props = {
  block: ContentBlock;
};

export function BlockRenderer({ block }: Props) {
  const site = useSite();
  const Component = site.blocks[block.type];

  if (!Component) {
    throw new Error(`Block type "${block.type}" is not registered`);
  }

  return <Component props={block.props} />;
}
