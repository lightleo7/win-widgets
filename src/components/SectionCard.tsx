import type { ReactNode } from "react";

type SectionCardProps = {
  children: ReactNode;
  className?: string;
};

export function SectionCard({
  children,
  className = "",
}: SectionCardProps) {
  return (
    <section
      className={`
        overflow-hidden
        rounded-3xl
        border
        border-white/[0.07]
        bg-[#121317]
        ${className}
      `}
    >
      {children}
    </section>
  );
}