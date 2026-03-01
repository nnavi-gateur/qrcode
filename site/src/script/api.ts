async function getData() {
  const response = await fetch("http://localhost:8000/hello");
  const data = await response.json();
  return data;
}

// Attendre que le DOM soit chargé avant d'accéder aux éléments
if (typeof document !== 'undefined') {
  const answer = document.getElementById("answer") as HTMLImageElement;
  const submit = document.getElementById("submit") as HTMLButtonElement;


  if (submit && answer) {
    submit.addEventListener("click", async function() {
      try {
        const response = await fetch("http://localhost:8000/qrcode");

        const data= await response.json();

        answer.src = "data:image/svg+xml;base64," + btoa(data.message);

    } catch (err) {
    console.error("Erreur complète:", err);
    answer.textContent = "Erreur";
  }
    }
    );
}}  