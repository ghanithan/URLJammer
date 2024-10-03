# URLJammer
URL shortener using the Jamstack concept

## Roadmap

- Create an URL shortening service that accepts a target URL and returns a shortened URL.
  - module to interact with the filesystem to create directories and files and encapsulate it with a feature flag
- Setup Axum server that serves static page and also exposes an API to shorten URL.
  - Encapsulate serving static pages with a feature flag
- Build a frontend to consume the API and test it locally
- Create an optional module that would replace the fileIO with AWS S3 interactions.
- Add JS code to collect the geolocation of the User along with basic metrics and accept them via an API (like a boomerang)
  - The collected metrics would be stored in a metrics-data file in the respective folders.
- Add a spawned task to fetch the page and spawn another 2 layer of tasks to take a screenshot of the page and another task to generate AI summary using AI.
  - The generated content would be stored along with the metrics-data file in the respective folders.
  - The Filesystem interactions should be replacable with a cloud storage interaction.
  - Add sections the redirect page to display the content.
- Split the above monolith into 3 services,
  - 1. Service to generate shortened URL and write to a cloud storage like AWS S3. (Since we would be using 3rd party AI service, it can be handled here )
  - 2. The static file rendering would be done through the cloud storage with a CDN front (AWS Cloudfront)
  - 3. Another serparate service to generate screenshot and store the same in cloud storage.
