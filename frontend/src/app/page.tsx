import { Container } from "@/components/ui/container";
import { Title } from "@/components/typography/title";

export default function Home() {
  return (
    <main className="w-screen min-h-screen flex justify-center items-center">
      <Container>
        <Title>Hello World</Title>
      </Container>
    </main>
  );
}
