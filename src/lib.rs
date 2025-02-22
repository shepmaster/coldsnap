// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*!
A library that uses the Amazon EBS direct APIs to work with snapshots.

# Examples

Downloading a snapshot into a disk image:
```
use coldsnap::SnapshotDownloader;
use rusoto_ebs::EbsClient;
use std::path::Path;

# async fn doc() {
let client = EbsClient::new(rusoto_core::Region::UsWest2);
let downloader = SnapshotDownloader::new(client);
let path = Path::new("./disk.img");

downloader.download_to_file("snap-1234", &path, None)
    .await
    .expect("failed to download snapshot");
# }
```

Uploading a disk image into a snapshot:
```
use coldsnap::SnapshotUploader;
use rusoto_ebs::EbsClient;
use std::path::Path;

# async fn doc() {
let client = EbsClient::new(rusoto_core::Region::UsWest2);
let uploader = SnapshotUploader::new(client);
let path = Path::new("./disk.img");

let snapshot_id = uploader.upload_from_file(&path, None, None, None)
        .await
        .expect("failed to upload snapshot");
# }
```

Waiting for a snapshot to be completed:
```
use coldsnap::SnapshotWaiter;
use rusoto_ec2::Ec2Client;

# async fn doc() {
let client = Ec2Client::new(rusoto_core::Region::UsWest2);
let waiter = SnapshotWaiter::new(client);

waiter.wait_for_completed("snap-1234")
        .await
        .expect("failed to wait for snapshot");
# }
```
*/

mod block_device;
mod download;
mod upload;
mod wait;

pub use download::Error as DownloadError;
pub use download::SnapshotDownloader;

pub use upload::Error as UploadError;
pub use upload::SnapshotUploader;

pub use wait::Error as WaitError;
pub use wait::{SnapshotWaiter, WaitParams};
