import clsx from "clsx";

export function Container({ children, className }: { 
    children: React.ReactNode, 
    className?: string 
}) {
    return (
      <div className="mx-auto w-full max-w-10xl px-4 sm:px-6 lg:px-8">
        <div className={clsx("mx-auto max-w-7xl", className)}>
            {children}
        </div>
      </div>
    )
  }
  
