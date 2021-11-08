using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Orbit : MonoBehaviour
{
    private float initAngle = 0;
    private float time = 0;
    private float gravitationalConstant = 6.674f * Mathf.Pow(10, -11);
    private float sma;
    public float eccentricity;


    Vector3 positionFromTime(float time) {
        BodyAttributes parentComponent = gameObject.transform.parent.gameObject.GetComponentInParent<BodyAttributes>();

        float planetMass = parentComponent.planetMass;

        float gravitaionalParameter = gravitationalConstant * planetMass;

        float period = 2.0f * Mathf.PI * Mathf.Sqrt(Mathf.Pow(sma, 3) / gravitaionalParameter);

        float meanMotion = 2.0f * Mathf.PI / period;
        float meanAnomaly = meanMotion * time;

        float eccentric_anomaly;

        if (eccentricity < 0.8) {
            eccentric_anomaly = meanAnomaly;
        } else {
            eccentric_anomaly = Mathf.PI;
        }

        float pseudo_true_anomaly = eccentric_anomaly - eccentricity * Mathf.Sin(meanAnomaly) - meanAnomaly;

        float delta = Mathf.Pow(10, -8);
        int i = 0;
        int iCap = 100;

        while ((Mathf.Abs(pseudo_true_anomaly) > delta) && (i < iCap)) {
            eccentric_anomaly = eccentric_anomaly - pseudo_true_anomaly / (1.0f - (eccentricity * Mathf.Cos(eccentric_anomaly)));
            pseudo_true_anomaly = eccentric_anomaly - eccentricity * Mathf.Sin(eccentric_anomaly) - meanAnomaly;
            i += 1;
        }

        // Trust me this is fine
        float true_anomaly = Mathf.Atan2(Mathf.Sqrt(1.0f - (Mathf.Pow(eccentricity, 2))) * Mathf.Sin(eccentric_anomaly), Mathf.Cos(eccentric_anomaly) - eccentricity);

        true_anomaly = true_anomaly + initAngle;

        float distance = sma * (1.0f - (eccentricity * Mathf.Cos(eccentric_anomaly)));

        float x_pos = distance * Mathf.Cos(true_anomaly);
        float y_pos = distance * Mathf.Sin(true_anomaly);

        return new Vector3(x_pos, y_pos, 0);
    }

    // Start is called before the first frame update
    void Start()
    {
        Vector3 parentPos = transform.parent.position;

        float distanceFromParent = Vector3.Distance(transform.position, parentPos);

        float angle = (Mathf.Atan2(transform.position.y - parentPos.y, transform.position.x - parentPos.x));

        initAngle = angle;

        sma = distanceFromParent * 1;
    }

    // Update is called once per frame
    void FixedUpdate()
    {
        time += Time.deltaTime;

        transform.position = transform.parent.position + positionFromTime(time);
    }
}
