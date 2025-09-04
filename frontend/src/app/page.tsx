import { Container } from "@/components/ui/container";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Subtitle } from "@/components/typography/subtitle";
import { Paragraph } from "@/components/typography/paragraph";

export default function Home() {
  return (
    <main className="w-screen min-h-screen flex justify-center items-center">
      <Container className="h-full grid grid-cols-1 grid-rows-4 md:grid-cols-4 md:grid-rows-1 gap-6">
        <Card className="aspect-square self-start">
          <CardHeader>
            <CardTitle>streak</CardTitle>
            <CardDescription>how consistent I was with writing daily notes</CardDescription>
          </CardHeader>
          <CardContent className="h-full flex justify-center items-center">
            <Subtitle className="!border-b-0 text-5xl">10</Subtitle>
          </CardContent>
        </Card>

        <Card className="row-span-4 min-h-132 md:row-span-1 md:col-span-3">
          <CardHeader>
            <CardTitle>notes</CardTitle>
            <CardDescription>latest written notes</CardDescription>
          </CardHeader>
          <CardContent className="space-y-12">
            <div className="space-y-3 mt-6">
              <div className="text-sm font-light">2025-09-05</div>
              <p>Lorem ipsum dolor sit amet consectetur adipisicing elit. Quisquam, quos.</p>
            </div>

            <div className="space-y-3">
              <div className="text-sm font-light">2025-09-04</div>
              <p>Lorem ipsum dolor sit amet consectetur adipisicing elit. Quisquam, quos.</p>
            </div>
          </CardContent>
        </Card>
      </Container>
    </main>
  );
}
