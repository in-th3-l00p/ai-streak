import clsx from "clsx";

export function Container({ children, className }: { 
    children: React.ReactNode, 
    className?: string 
}) {
    return (
      <div className="mx-auto w-full max-w-8xl px-3 sm:px-6 lg:px-8">
        <div className={clsx("mx-auto max-w-5xl", className)}>
            {children}
        </div>
      </div>
    )
  }
  
