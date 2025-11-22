// Main JavaScript file
console.log("TouchCalc loaded");

// Helper function to show error messages
function showError(elementId, message) {
  const errorElement = document.getElementById(elementId);
  if (errorElement) {
    errorElement.textContent = message;
    errorElement.style.display = "block";
    setTimeout(() => {
      errorElement.style.display = "none";
    }, 5000);
  }
}

// Helper function to show success messages
function showSuccess(elementId, message) {
  const successElement = document.getElementById(elementId);
  if (successElement) {
    successElement.textContent = message;
    successElement.style.display = "block";
    setTimeout(() => {
      successElement.style.display = "none";
    }, 5000);
  }
}

// Check authentication status
async function checkAuth() {
  try {
    const response = await fetch("/api/auth/me", {
      credentials: "include",
    });
    return response.ok;
  } catch (error) {
    return false;
  }
}
