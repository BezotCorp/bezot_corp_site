type Props = {
  props?: Readonly<Record<string, unknown>>;
};

export function MailLinkBlock({ props }: Props) {
  const email = typeof props?.email === "string" ? props.email : "";
  const label = typeof props?.label === "string" ? props.label : email;

  if (!email) {
    return null;
  }

  return (
    <section>
      <a href={`mailto:${email}`}>{label}</a>
    </section>
  );
}
