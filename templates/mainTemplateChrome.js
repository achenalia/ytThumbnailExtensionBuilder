(() => {
    const imageFilePath = "./assets/images/"; //images should generally be size 664px x 365px and have a transparent background
    const numImages = 0; //change this number to change the number of images
    const flipRandomPercent = 2; //this number represents how many numbers to randomly choose. bigger = less likely, smaller = more likely.
    var isEnabled = true;

    // gets all YouTube thumbnails on the page
    function getThumbnails() {
        const thumbnailQuery = "ytd-thumbnail:not(.ytd-video-preview, .ytd-rich-grid-slim-media) a > yt-image > img.yt-core-image:only-child:not(.yt-core-attributed-string__image-element),.ytp-videowall-still-image:not([style*='extension:'])";

        const thumbnail = document.querySelectorAll(thumbnailQuery);

        thumbnail.forEach((image) => {
            let counter = Math.random() > 0.001 ? 1 : 20;
            let i = 0;
            for (i = 0; i < counter; i++) {
                const index = getRandomImage();
                let flip = getImageState(index);
                let url = getImageURL(index);
                applyThumbnails(image, url, flip);
            }
        });
    }
    
    //returns the url of an image
    function getImageURL(index) {
        return chrome.runtime.getURL(`${imageFilePath}${index}.png`);
    }

    //applies the thumbnail images to the thumbnails
    function applyThumbnails(image, imageUrl, flip = false) {
        if (image.nodeName == "IMG") {
            const overlay = document.createElement("img");
            overlay.src = imageUrl;
            overlay.style.position = "absolute";
            overlay.style.top = "1%"; // prevents the image from being slightly above the bottom of the frame
            overlay.style.left = getRandomPercent(35) + "%"; // randomly positions the image from -35% to 35% of the frame, adjust if you'd like
            overlay.style.width = "100%";
            overlay.style.height = "100%";
            overlay.style.zIndex = "0";
            overlay.style.opacity = "1.0";
            

            if(flip)
                {
                    overlay.style.transform = "scaleX(-1)"; //flips the image
                }

            image.style.position = "relative";
            image.parentElement.appendChild(overlay);
        } else if (image.nodeName == "DIV") {
            image.style.backgroundImage = `url("${imageUrl}"), ` + image.style.backgroundImage;
        }
    }

    function getRandomInt(max) {
        return Math.floor(Math.random() * max);
    }

    function getRandomPercent(percent) {
        return Math.floor(Math.random() * (2 * percent + 1)) - percent;
    }
    
    // decides whether to flip image
    function getImageState()
    {
        // percent to flip is default 50% when flipRandomPercent = 2

        let random = 0;
        random = getRandomInt(flipRandomPercent); // returns a random number from 0 to flipRandomPercent

        if(random === 1)
        {
            return true; //STATE: flip image
        }
        else
        {
            return false; //STATE: do not flip image
        }

    }

    //returns a random image
    function getRandomImage() {
        let random = getRandomInt(numImages) + 1; // returns a number between 1 and numImages
        return random;
    }

    //runs the functions
    if (isEnabled) {
        setInterval(getThumbnails, 200);
    }
})();
