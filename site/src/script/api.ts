

const answer = document.getElementById("answer") as HTMLImageElement;
const downloadLink = document.getElementById("download-link") as HTMLAnchorElement;
const submit = document.getElementById("submit") as HTMLButtonElement;


let security =document.getElementById('security-level') as HTMLSelectElement;
if (security){
  security.selectedIndex = 0;
}

if (submit && answer) {
  submit.addEventListener("click", async function() {
    try {
      const content = (document.getElementById("qr-input") as HTMLInputElement).value;
      const encodedContent = encodeURIComponent(content);  // Add this
      const format = (document.getElementById("format") as HTMLSelectElement).value;
      let security =document.getElementById('security-level') as HTMLSelectElement;

      let data:any;
      const apiBase = import.meta.env.PUBLIC_API_URL ?? "http://localhost:8000";
      if (format === "jpg") {
        const response = await fetch(`${apiBase}/qrcode/JPG/${encodedContent}/${security.selectedIndex}`);
        data= await response.json();
        answer.src = "data:image/jpeg;base64," + data.message;
        downloadLink.href = "data:image/jpeg;base64," + data.message;
        downloadLink.style.display = "block";
      }
      else{
        const response = await fetch(`${apiBase}/qrcode/SVG/${encodedContent}/${security.selectedIndex}`);

        data= await response.json();
        answer.src = "data:image/svg+xml;base64," + btoa(data.message);
        downloadLink.href = "data:image/svg+xml;base64," + btoa(data.message);
        downloadLink.style.display = "block";
        
      }

        } catch (err) {
          console.error("Erreur complète:", err);
          answer.textContent = "Erreur";
          }
   } );
}


