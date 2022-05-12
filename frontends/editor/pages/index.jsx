import { useState } from 'react';

function Header({ title }) {
  return <h1>{title ? title : 'Forjot: Unknown'}</h1>;
}

export default function HomePage() {
  return (
    <div>
      <Header title="Fornjot: Test Page" />
    </div>
  );
}
