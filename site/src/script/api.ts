
const answer = document.getElementById("answer") as HTMLImageElement;
const downloadLink = document.getElementById("download-link") as HTMLAnchorElement;
const submit = document.getElementById("submit") as HTMLButtonElement;
const shortenCheckbox = document.getElementById("shorten-checkbox") as HTMLInputElement;
const formatJpgBtn = document.getElementById("format-jpg") as HTMLButtonElement;
const formatSvgBtn = document.getElementById("format-svg") as HTMLButtonElement;
const sec_select=document.getElementById("sec-select") as HTMLParagraphElement;

let currentFormat = "jpg";

let security = document.getElementById('security-level') as HTMLSelectElement;
if (security) {
  security.selectedIndex = 0;
}

// Download link reset on click
if (downloadLink) {
  downloadLink.addEventListener("click", function() {
      setTimeout(function() {
          downloadLink.style.color = "rgb(235, 234, 234)";
          downloadLink.style.backgroundColor = "#121212";
          downloadLink.style.border = "2px solid rgb(43, 41, 41)";
          downloadLink.blur();
      }, 8000);
  });
}
if (formatJpgBtn) {
  formatJpgBtn.addEventListener("click", function() {
    currentFormat = "jpg";
    formatJpgBtn.classList.add("active");
    formatSvgBtn?.classList.remove("active");
  });
}

if (formatSvgBtn) {
  formatSvgBtn.addEventListener("click", function() {
    currentFormat = "svg";
    formatSvgBtn.classList.add("active");
    formatJpgBtn?.classList.remove("active");
  });
}

if (submit && answer) {
  submit.addEventListener("click", async function() {
    try {
      const apiBase = "/api";
      let content = (document.getElementById("qr-input") as HTMLInputElement).value;

      // If the "Shorten URL" checkbox is checked, shorten the URL first.
      if (shortenCheckbox && shortenCheckbox.checked) {
        const shortenResp = await fetch(`${apiBase}/shorten`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ url: content }),
        });

        if (!shortenResp.ok) {
          const errText = await shortenResp.text();
          console.error("Shortening failed:", errText);
          return;
        }

        const shortenData = await shortenResp.json();
        content = shortenData.short_url as string;
      }

      const encodedContent = encodeURIComponent(content);
      const securityEl = document.getElementById('security-level') as HTMLSelectElement;
      if (securityEl.selectedIndex == 0){
          sec_select.style.display = "block";
          setTimeout (() => {sec_select.style.display = "none";}, 4000);
          return;
      }

      let data: any;
      if (currentFormat === "jpg") {
        const reponse = await fetch(`${apiBase}/qrcode/JPG?content=${encodedContent}&level=${securityEl.selectedIndex}`);
        data = await reponse.json();
        answer.src = "data:image/jpeg;base64," + data.message;
        downloadLink.href = "data:image/jpeg;base64," + data.message;
        downloadLink.style.display = "block";
      } else {
        const reponse = await fetch(`${apiBase}/qrcode/SVG?content=${encodedContent}&level=${securityEl.selectedIndex}`);
        data = await reponse.json();
        answer.src = "data:image/svg+xml;base64," + btoa(data.message);
        downloadLink.href = "data:image/svg+xml;base64," + btoa(data.message);
        downloadLink.style.display = "block";
      }

      // Reset download link to initial state after 4 seconds
      setTimeout(() => {
        downloadLink.style.backgroundColor = "#121212";
        downloadLink.style.color = "rgb(235, 234, 234)";
        downloadLink.style.border = "2px solid rgb(43, 41, 41)";
      }, 4000);

    } catch (err) {
      console.error("Erreur complète:", err);
      answer.textContent = "Erreur";
    }
  });
}
