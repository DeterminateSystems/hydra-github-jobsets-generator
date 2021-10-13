terraform {
  required_providers {
    hydra = {
      version = "~> 0.1"
      source  = "DeterminateSystems/hydra"
    }
  }
}

provider "hydra" {
  host = "http://127.0.0.1:63333/"
  username = "alice"
  password = "foobar"
}

