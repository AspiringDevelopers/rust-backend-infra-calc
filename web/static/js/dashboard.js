// Dashboard functionality
document.addEventListener("DOMContentLoaded", async () => {
  // Check if user is authenticated
  const isAuth = await checkAuth();
  if (!isAuth) {
    window.location.href = "/login";
    return;
  }

  // Load user files
  loadFiles();
});

async function loadFiles() {
  try {
    const response = await fetch("/api/files", {
      credentials: "include",
    });

    if (response.ok) {
      const files = await response.json();
      displayFiles(files);
    } else {
      document.getElementById("files-list").innerHTML =
        "<p>Error loading files</p>";
    }
  } catch (error) {
    document.getElementById("files-list").innerHTML =
      "<p>Error loading files</p>";
  }
}

function displayFiles(files) {
  const filesList = document.getElementById("files-list");

  if (files.length === 0) {
    filesList.innerHTML = "<p>No files found</p>";
    return;
  }

  const html = files
    .map(
      (file) => `
        <div class="file-item">
            <span>${file.path}</span>
            <span>${new Date(file.updated_at).toLocaleDateString()}</span>
        </div>
    `
    )
    .join("");

  filesList.innerHTML = html;
}
