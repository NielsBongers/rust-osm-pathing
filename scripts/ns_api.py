
import json
import json.decoder
import urllib.request
from datetime import datetime

SUBSCRIPTION_KEY = "3cce8b71a9c94892bce40ba5d7d05593"

from_station = "Eindhoven Strijp-S"
to_station = "Den Haag Moerwijk"
time = datetime.now().isoformat()


try:
    url = f"https://gateway.apiportal.ns.nl/reisinformatie-api/api/v3/trips?fromStation={from_station}&toStation={to_station}&originWalk=false&originBike=false&originCar=false&destinationWalk=false&destinationBike=false&destinationCar=false&dateTime={time}&shorterChange=false&travelAssistance=false&searchForAccessibleTrip=false&localTrainsOnly=false&excludeHighSpeedTrains=false&excludeTrainsWithReservationRequired=false&discount=NO_DISCOUNT&travelClass=2&passing=false&travelRequestType=DEFAULT".replace(" ", "%20")

    hdr ={
    'Cache-Control': 'no-cache',
    'Ocp-Apim-Subscription-Key': SUBSCRIPTION_KEY
    }

    req = urllib.request.Request(url, headers=hdr)

    req.get_method = lambda: 'GET'
    response = urllib.request.urlopen(req)
    
    data = json.loads(response.read())

    trips = data["trips"]
    
    for trip in data["trips"]: 
        actual_duration = trip["actualDurationInMinutes"]
        status = trip["status"]
        
        for legs in trip["legs"]: 
            planned_departure = legs["origin"]["plannedDateTime"]
            
            print(planned_departure)
        
        # print(f"Trip with {actual_duration} min and {status}")
    
except Exception as e:
    print(e)