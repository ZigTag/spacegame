using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class CameraMovement : MonoBehaviour
{
    Vector3 cameraOffset;
    float cameraSpeed = 0.1F;
    // Start is called before the first frame update
    void Start()
    {
        transform.position = transform.position + cameraOffset;
        
    }

    // Update is called once per frame
    void FixedUpdate()
    {
        Vector3 targetedPosition = new Vector3(0, 0, -10);
        Vector3 parentComponentPosition = new Vector3(0, 0, -10);
        TargetableObject[] targets = FindObjectsOfType<TargetableObject>();

        for (int i = 0; i < targets.Length; i++)
        {
            TargetableObject tObject = targets[i];

            if (tObject.targeted) {
                targetedPosition = tObject.transform.position;

                if (tObject.GetComponent<SystemHost>() == null)
                {
                    parentComponentPosition = tObject.transform.parent.gameObject.GetComponentInParent<BodyAttributes>().transform.position;
                }
            }
        }

        float horizontalSpeed = .005F;
        float verticalSpeed = .005F;

        float xMouse = Input.mousePosition.x - (Screen.width / 2);
        float yMouse = Input.mousePosition.y - (Screen.height / 2);

        float xMovement = horizontalSpeed * xMouse;
        float yMovement = verticalSpeed * yMouse;

        Vector3 calculatedMousePos = new Vector3(xMovement, yMovement, -10);

        Vector3 finalPosition = calculatedMousePos + cameraOffset + Vector3.Lerp(targetedPosition, parentComponentPosition, 0.45F);
        Vector3 lerpPosition = Vector3.Lerp(transform.position, finalPosition, cameraSpeed);
        transform.position = lerpPosition;
    }
}
