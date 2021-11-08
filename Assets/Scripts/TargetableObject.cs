using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class TargetableObject : MonoBehaviour
{
    public bool targeted;

    // Activates when mouse is hovered over box collider
    void OnMouseOver()
    {
        if (Input.GetMouseButtonDown(0))
        {
            TargetableObject[] targets = FindObjectsOfType<TargetableObject>();

            print(targets.Length);

            for (int i = 0; i < targets.Length; i++)
            {   
                if (targets[i].targeted) {
                    targets[i].targeted = false;
                }
            }

            targeted = true;
        }
    }
}
