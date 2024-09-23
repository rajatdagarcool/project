// Function to display an alert when the "Contact Us" button is clicked
function contactUs() {
    alert("Thank you for reaching out! We'll contact you soon.");
}

// Function to dynamically change text content in the about section
function updateAboutUs() {
    const aboutSection = document.querySelector('section p');
    aboutSection.textContent = "Our updated mission is to provide excellent web development services.";
}

// Event listener for the "Contact Us" button
document.addEventListener('DOMContentLoaded', () => {
    const contactButton = document.querySelector('button');
    contactButton.addEventListener('click', contactUs);

    // Optionally, update the About Us text after a delay
    setTimeout(updateAboutUs, 5000);  // Update text after 5 seconds
});

