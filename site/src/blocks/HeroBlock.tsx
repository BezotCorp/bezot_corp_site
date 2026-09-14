type Props = {
  props?: Readonly<Record<string, unknown>>;
};

export function HeroBlock({ props }: Props) {
  const title = typeof props?.title === "string" ? props.title : "";
  const subtitle = typeof props?.subtitle === "string" ? props.subtitle : "";

  return (
    <section>
      <h1>{title}</h1>
      {subtitle && <p>{subtitle}</p>}
    </section>
  );
}
