use std::fs::File;
use std::io::(self,ErrorKind);
use std::io::Read;

fn fetch_random_dog_image() -> ApiResult {
    let url = "https://dog.ceo/api/breeds/image/random";
    
    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                match response.into_json::<DogImage>() {
                    Ok(dog_image) => ApiResult::Success(dog_image),
                    Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                }
            } else {
                ApiResult::ApiError(format!("HTTP error: {}", response.status()))
            }
        },
        Err(e) => {
            let error_details = format!("Request failed: {}", e);
            ApiResult::NetworkError(error_details)
        },
    }
}

fn main(){
    fetch_random_dog_image();
}