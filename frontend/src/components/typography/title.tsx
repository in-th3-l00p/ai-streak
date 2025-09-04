import { cn } from "@/lib/utils";

export function Title({ 
    children,
    className,  
}: { children: React.ReactNode, className?: string }) {
  return (
    <h1 className={cn("scroll-m-20 text-center text-4xl font-semibold tracking-tight text-balance", className)}>
      {children}
    </h1>
  )
}
