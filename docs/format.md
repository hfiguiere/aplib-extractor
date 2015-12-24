Aperture Library format
=======================




App version | DB version | DB minor | Project vers
------------+------------+----------+-------------
3.1.3       | 110        | 122      | 6
3.2.2       | 110        | 131      | 7
3.3.2       | 110        | 207 (203)| 8
3.4.5       | 110        | 219 (208)| 8


Bundle structure
----------------

This is the file structure of the bundle for version 110. Between [ ]
is the earliest DB minor we *saw* the file.


Aperture.aplibrary
|
+- Aperture.aplib
|  +- DataModelVersion.plist
|
+- ApertureData.xml [131]
+- Attachments
+- Database
|  +- ActiveWebPublishingAccounts.plist [207]
|  +- Albums
|  |  +- *.aplbum
|  |  +-
|  |
|  +- apdb
|  |  +- BigBlobs.apdb
|  |  +- Faces.db
|  |
|  +- BigBlobs.apdb -> apdb/BigBlobs.apdb
|  +- DataModelVersion.plist
|  +- Faces
|  |  +- Detected
|  |     +- *.apdetected
|  |  +- DetectedExternals
|  |  +- FaceExternals
|  |  +- FaceNames
|  |
|  +- Faces.db -> apdb/Faces.db
|  +- Folders
|  |  +- *.apfolder
|  |
|  +- History
|  |  +- Changes
|  |     +- *.plist
|  |
|  +- History.apdb -> apdb/History.apdb
|  +- ImageProxies.apdb -> apdb/ImageProxies.apdb
|  +- KeywordSets.plist [207]
|  +- Keywords.plist
|  +- Library.apdb -> apdb/Library.apdb
|  +- Places
|  |  +- *.applace
|  |
|  +- Properties.apdb -> apdb/Properties.apdb
|  +- tmSync.plist
|  +- Vaults
|  +- Versions
|  |  +- YYYY
|  |     +- MM
|  |        +- DD
|  |           +- YYYYMMDD-nnnnnn
|  |              +- <id>
|  |                 +- Master.apmaster
|  |                 +- Version-0.apversion
|  |                 +- Version-1.apversion
|  |
|  +- Volumes
|
+- iLifeShared
|  +- ApertureDatabaseTimestamp
|
+- Info.plist
+- Masks
+- Masters
|  +- YYYY
|     +- MM
|        +- DD
|           +- YYYYMMDD-nnnnnn
|             +- *
|
+- Previews
+- Thumbnails



ApertureData.xml: seems to contain a dump of the whole data model but
it seems to not be present everywhere.

Database/Folders is all the containers, folders, project, etc.
The .apfolder files are binary plists.



Folders
-------

